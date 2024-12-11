import React, { useState } from 'react';
import axios from 'axios';

const FileUpload = () => {
  const [selectedFiles, setSelectedFiles] = useState([]);
  const [uploadPercentage, setUploadPercentage] = useState(0);

  const handleFileSelect = (event) => {
    setSelectedFiles(event.target.files);
    setUploadPercentage(0); // Reset progress indicator on new file(s) selection
  };

  const handleFileUpload = async () => {
    if (!selectedFiles.length) {
      alert('Please select file(s) first!');
      return;
    }

    const formData = new FormData();

    // Validate file size and append files to FormData
    for (const file of selectedFiles) {
      if (file.size > 10 * 1024 * 1024) {  // Example size limit: 10MB
        alert(`${file.name} is too large. Maximum size is 10MB.`);
        return;
      }
      formData.append('files', file);
    }

    try {
      const response = await axios.post(process.env.REACT_APP_UPLOAD_URL, formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
        onUploadProgress: (progressEvent) => {
          let percentCompleted = Math.round((progressEvent.loaded * 100) / progressEvent.total);
          setUploadPercentage(percentCompleted);
        }
      });

      alert('File(s) uploaded successfully');
      console.log('Server Response:', response.data);
    } catch (error) {
      alert('File upload failed');
      console.error('Error uploading file(s):', error);
    }
  };

  const progressBar = uploadPercentage > 0 ? (
    <div style={{ width: '100%', backgroundColor: '#ddd' }}>
      <div
        style={{
          height: '20px',
          backgroundColor: 'green',
          width: `${uploadPercentage}%`,
        }}
      >
        {uploadPercentage}%
      </div>
    </div>
  ) : null;

  return (
    <div style={{ margin: '20px' }}>
      <h2>Upload file(s)</h2>
      <input type="file" multiple onChange={handleFileSelect} />
      <button onClick={handleFileUpload}>Upload</button>
      {progressBar}
    </div>
  );
};

export default FileUpload;