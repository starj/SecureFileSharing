import React, { useState, useEffect } from 'react';
import axios from 'axios';

const UploadedFiles = () => {
    const [files, setFiles] = useState([]);

    const fetchFiles = async () => {
        try {
            const response = await axios.get(`${process.env.REACT_APP_BACKEND_URL}/files`);
            setFiles(response.data);
        } catch (error) {
            console.error("Failed to fetch files:", error);
        }
    };

    useEffect(() => {
        fetchFiles();
    }, []);

    return (
        <div>
            <h2>Uploaded Files</h2>
            <ul>
                {files.map((file, index) => (
                    <li key={index}>{file.name}</li>
                ))}
            </ul>
        </div>
    );
};

export default UploadedFiles;